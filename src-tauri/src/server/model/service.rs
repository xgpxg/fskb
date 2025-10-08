use crate::common::id;
use crate::common::req::IdReq;
use crate::db;
use crate::db::model::model::{Model, ModelBuilder, ModelSource, ModelStatus};
use crate::db::{tools, Pool};
use crate::server::model::request::{ModelAddReq, ModelUpdateReq};
use crate::server::model::response::{ModelListRes, ModelSimpleListRes, OfflineModelListRes};
use anyhow::bail;
use rbatis::RBatis;
use rbs::value;

pub(crate) async fn list_all() -> anyhow::Result<Vec<ModelListRes>> {
    let list = Model::select_all(Pool::get()?).await?;
    let list = list
        .into_iter()
        .map(|item| ModelListRes {
            id: item.id,
            name: item.name,
            description: item.description,
            source: item.source,
            icon: item.icon,
            status: item.status,
            status_msg: item.status_msg,
            base_url: item.base_url,
            api_key: item.api_key,
            max_token: item.max_token,
            task_type: item.task_type,
            create_time: item.create_time,
            update_time: item.update_time,
        })
        .collect::<Vec<_>>();
    Ok(list)
}

pub(crate) async fn add(req: ModelAddReq) -> anyhow::Result<()> {
    // 检查名称是否重复
    let count = db::model::model::check_name_exists(Pool::get()?, &req.name, None).await?;
    if *count > 0 {
        bail!("模型名称重复");
    }
    let model = Model {
        id: Some(id::next()),
        name: Some(req.name),
        description: req.description,
        source: Some(ModelSource::Custom as i8),
        icon: req.icon,
        status: Some(ModelStatus::Disable as i8),
        status_msg: None,
        base_url: Some(req.base_url),
        api_key: req.api_key,
        max_token: req.max_token,
        task_type: Some(req.task_type),
        create_user_id: None,
        update_user_id: None,
        create_time: Some(tools::now()),
        update_time: None,
        remark: None,
        user_id: None,
        is_delete: None,
    };

    Model::insert(Pool::get()?, &model).await?;

    Ok(())
}

pub(crate) async fn update(req: ModelUpdateReq) -> anyhow::Result<()> {
    let tx = Pool::get()?;
    // 检查模型是否存在
    let model = Model::select_by_map(tx, value! {"id": &req.id}).await?;
    if model.is_empty() {
        bail!("模型不存在");
    }
    if let Some(ref name) = req.name {
        if name.trim().is_empty() {
            bail!("模型名称不能为空");
        }
        // 检查名称是否重复
        let count = db::model::model::check_name_exists(tx, name, Some(req.id)).await?;
        if *count > 0 {
            return bail!("模型名称重复");
        }
    }

    let model = model.first().unwrap();
    // 如果是更新状态
    if let Some(status) = req.status {
        // 如果是启用状态，检查API是否配置
        if status == ModelStatus::Enable as i8 {
            if model.base_url.is_none() || model.base_url.as_ref().unwrap().trim() == "" {
                bail!("API未配置，无法启用模型");
            }
            // 如果是内置模型，api key必须配置
            if model.source == Some(ModelSource::Inner as i8) {
                if model.api_key.is_none() || model.api_key.as_ref().unwrap().trim() == "" {
                    bail!("API KEY未配置，无法启用内置模型");
                }
            }
        }
    }

    let model_for_update = ModelBuilder::default()
        .id(Some(req.id))
        .name(req.name)
        .description(req.description)
        .icon(req.icon)
        .base_url(req.base_url)
        .api_key(req.api_key)
        .max_token(req.max_token)
        .status(req.status)
        .task_type(req.task_type)
        .update_time(Some(tools::now()))
        .build()?;

    Model::update_by_map(tx, &model_for_update, value! {"id": &req.id}).await?;

    Ok(())
}

pub(crate) async fn delete(req: IdReq) -> anyhow::Result<()> {
    let model = Model::select_by_map(Pool::get()?, value! {"id": &req.id}).await?;
    if model.is_empty() {
        bail!("模型不存在");
    }
    let model = model.first().unwrap();
    // 启用状态不可删除
    if model.status == Some(ModelStatus::Enable as i8) {
        bail!("模型已启用，请先停用后删除");
    }
    Model::delete_by_map(Pool::get()?, value! {"id": &req.id}).await?;

    Ok(())
}

pub(crate) async fn available() -> anyhow::Result<Vec<ModelSimpleListRes>> {
    let list = Model::select_by_map(
        Pool::get()?,
        value! {
           "status": ModelStatus::Enable as i8,
            "is_delete": 0,
            "task_type": 1,
        },
    )
    .await?;
    let list = list
        .into_iter()
        .map(|item| ModelSimpleListRes {
            id: item.id,
            name: item.name,
            description: item.description,
            icon: item.icon,
        })
        .collect::<Vec<_>>();
    Ok(list)
}

pub async fn get_model_by_name(model_name: &str) -> anyhow::Result<Model> {
    let model = Model::select_by_map(Pool::get()?, value! {"name": model_name}).await?;
    if model.is_empty() {
        bail!("模型不存在");
    }
    Ok(model.first().unwrap().clone())
}

pub(crate) async fn all_available(
    task_type: Option<i8>,
) -> anyhow::Result<Vec<ModelSimpleListRes>> {
    let list = Model::select_by_map(
        Pool::get()?,
        value! {
           "status": ModelStatus::Enable as i8,
            "is_delete": 0,
            "task_type": task_type,
        },
    )
    .await?;
    let list = list
        .into_iter()
        .map(|item| ModelSimpleListRes {
            id: item.id,
            name: item.name,
            description: item.description,
            icon: item.icon,
        })
        .collect::<Vec<_>>();
    Ok(list)
}

pub(crate) async fn list_all_offline_models() -> anyhow::Result<Vec<OfflineModelListRes>> {
    let offline_models = model::ModelManager::list().await?;

    let list = Model::select_all(Pool::get()?).await?;

    let offline_models = offline_models
        .iter()
        .map(|item| {
            let m = list.iter().find(|v| v.name.as_ref() == Some(&item.name));
            let status = if let Some(m) = m {
                m.status.unwrap()
            } else {
                // -1代表未安装
                -1
            };
            OfflineModelListRes {
                name: item.name.clone(),
                summary: item.summary.clone(),
                description: item.description.clone(),
                status,
                task_type: item.task_type,
            }
        })
        .collect::<Vec<_>>();

    Ok(offline_models)
}

pub(crate) async fn install_offline_model(name: String) -> anyhow::Result<()> {
    let offline_model = model::ModelManager::get(&name).await?;
    let tx = Pool::get()?;
    // 生成一条记录，状态为安装中
    let model = ModelBuilder::default()
        .id(Some(id::next()))
        .name(Some(name.clone()))
        .status(Some(ModelStatus::Installing as i8))
        .task_type(Some(offline_model.task_type))
        .base_url(Some("".to_string()))
        .source(Some(ModelSource::Local as i8))
        .create_time(Some(tools::now()))
        .build()?;

    Model::insert(tx, &model).await?;

    // 执行安装
    // 注意这里需要异步执行，否则前端刷新后会重复进入该方法，导致执行2次
    // 应该是tauri影响的
    tokio::spawn(async move {
        let res = model::ModelManager::install(&name).await;
        let tx = Pool::get()?;
        if let Err(e) = res {
            update_model_status(tx, &name, ModelStatus::Error, &e.to_string()).await?;
            bail!("模型安装失败：{}", e);
        }

        // 更新状态为未启用
        update_model_status(tx, &name, ModelStatus::Disable, "").await?;

        Ok::<(), anyhow::Error>(())
    });

    Ok(())
}

async fn update_model_status(
    tx: &RBatis,
    name: &str,
    status: ModelStatus,
    msg: &str,
) -> anyhow::Result<()> {
    let update = ModelBuilder::default()
        .name(Some(name.to_string()))
        .status(Some(status as i8))
        .status_msg(Some(msg.to_string()))
        .build()?;
    Model::update_by_map(
        tx,
        &update,
        value! {
            "name": name,
        },
    )
    .await?;

    Ok(())
}

pub(crate) async fn uninstall_offline_model(name: String) -> anyhow::Result<()> {
    let tx = Pool::get()?;
    let model = Model::select_by_map(
        tx,
        value! {
            "name": &name,
        },
    )
    .await?;
    if model.is_empty() {
        return bail!("模型不存在");
    }
    let model = model.first().unwrap();
    stop_offline_model(name.clone()).await?;
    delete(IdReq {
        id: model.id.unwrap(),
    })
    .await?;
    model::ModelManager::stop(&name).await?;
    model::ModelManager::uninstall(&name).await
}

pub(crate) async fn run_offline_model(name: String) -> anyhow::Result<()> {
    let tx = Pool::get()?;
    let model = Model::select_by_map(
        tx,
        value! {
            "name": &name,
        },
    )
    .await?;
    if model.is_empty() {
        return bail!("模型不存在");
    }

    update_model_status(tx, &name, ModelStatus::Starting, "").await?;

    let api = model::ModelManager::start(&name).await?;

    let update = ModelBuilder::default()
        .name(Some(name.clone()))
        .base_url(Some(api))
        .build()?;
    Model::update_by_map(
        tx,
        &update,
        value! {
            "name": &name,
        },
    )
    .await?;

    update_model_status(tx, &name, ModelStatus::Enable, "").await?;

    Ok(())
}

pub(crate) async fn stop_offline_model(name: String) -> anyhow::Result<()> {
    let res = model::ModelManager::stop(&name).await;

    if let Err(e) = res {
        update_model_status(Pool::get()?, &name, ModelStatus::Error, &e.to_string()).await?;
    }

    update_model_status(Pool::get()?, &name, ModelStatus::Disable, "").await?;
    Ok(())
}
pub(crate) async fn start_offline_model_on_start() {
    Model::select_by_map(
        Pool::get().unwrap(),
        value! {
            "source" :ModelSource::Local as i8,
            "status": ModelStatus::Enable as i8,
        },
    )
    .await
    .unwrap()
    .into_iter()
    .for_each(|model| {
        tokio::spawn(async move {
            run_offline_model(model.name.unwrap()).await.unwrap();
        });
    });
}
