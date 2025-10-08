use crate::profile_extract;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use rand::RngCore;
use std::fs;
use std::path::PathBuf;

const USER_PROFILE_ENCRYPT_KEY: &str = env!("USER_PROFILE_ENCRYPT_KEY");

/// 用户画像
pub struct UserProfile {
    ///  画像数据，yaml格式
    data: String,
}
impl Default for UserProfile {
    fn default() -> Self {
        Self {
            data: include_str!("../user_profile_template.yml").to_string(),
        }
    }
}
impl UserProfile {
    /// 从指定位置加载用户画像
    #[cfg(debug_assertions)]
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        Ok(Self {
            data: fs::read_to_string(path)?,
        })
    }

    #[cfg(not(debug_assertions))]
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read(path)?;
        let key_bytes: [u8; 32] = USER_PROFILE_ENCRYPT_KEY.as_bytes()[0..32]
            .try_into()
            .map_err(|_| anyhow::anyhow!("Key must be at least 32 bytes long"))?;
        let profile = Self::decrypt(&content, &key_bytes)?;
        Ok(Self { data: profile })
    }

    /// 获取用户画像
    pub fn get(&self) -> String {
        self.data.clone()
    }

    /// 从用户消息和助手消息中提取用户画像
    pub async fn extract(
        &mut self,
        user_message: &str,
        assistant_message: &str,
        base_url: &str,
        model_name: &str,
        api_key: &str,
    ) -> anyhow::Result<()> {
        let profile = profile_extract::extra(
            user_message,
            assistant_message,
            &self.data.clone(),
            base_url,
            model_name,
            api_key,
        )
        .await?;
        if let Some(profile) = profile {
            self.data = profile;
        }
        Ok(())
    }

    /// 保存用户画像
    #[cfg(debug_assertions)]
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        fs::write(path, self.data.clone())?;
        Ok(())
    }

    #[cfg(not(debug_assertions))]
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let key_bytes: [u8; 32] = USER_PROFILE_ENCRYPT_KEY.as_bytes()[0..32]
            .try_into()
            .map_err(|_| anyhow::anyhow!("Key must be at least 32 bytes long"))?;
        let profile = self.encrypt(&key_bytes)?;
        fs::write(path, profile)?;
        Ok(())
    }

    fn encrypt(&self, key: &[u8; 32]) -> anyhow::Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key)?;

        // 生成随机nonce
        let mut nonce_bytes = [0u8; 12];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // 加密数据
        let ciphertext = cipher
            .encrypt(nonce, self.data.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // 组合nonce和密文
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    fn decrypt(encrypted_data: &[u8], key: &[u8; 32]) -> anyhow::Result<String> {
        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Invalid encrypted data"));
        }

        let nonce = Nonce::from_slice(&encrypted_data[0..12]);
        let ciphertext = &encrypted_data[12..];

        let cipher = Aes256Gcm::new_from_slice(key)?;
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        Ok(String::from_utf8(plaintext)?)
    }
}
