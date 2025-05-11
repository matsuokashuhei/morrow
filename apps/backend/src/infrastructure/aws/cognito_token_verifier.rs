use anyhow::{Result, format_err};
use async_trait::async_trait;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use jsonwebtokens_cognito::KeySet;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::repositories::token_verifier::{TokenClaims, TokenVerifier};

#[derive(Clone)]
pub struct CognitoTokenVerifier {
    region: String,
    user_pool_id: String,
    client_id: String,
    jwks: Option<HashMap<String, DecodingKey>>,
    http_client: Client,
}

impl CognitoTokenVerifier {
    pub fn new(region: String, user_pool_id: String, client_id: String) -> Self {
        Self {
            region,
            user_pool_id,
            client_id,
            jwks: None,
            http_client: Client::new(),
        }
    }

    // JWKSエンドポイントからキーを取得
    async fn fetch_jwks(&mut self) -> Result<()> {
        let jwks_url = format!(
            "https://cognito-idp.{}.amazonaws.com/{}/.well-known/jwks.json",
            self.region, self.user_pool_id
        );

        #[derive(Deserialize)]
        struct JwkKey {
            kid: String,
            n: String,
            e: String,
            #[serde(rename = "alg")]
            algorithm: String,
            kty: String,
            #[serde(rename = "use")]
            key_use: String,
        }

        #[derive(Deserialize)]
        struct Jwks {
            keys: Vec<JwkKey>,
        }

        let jwks: Jwks = self.http_client.get(&jwks_url).send().await?.json().await?;

        let mut keys_map = HashMap::new();
        for key in jwks.keys {
            if key.key_use == "sig" && key.kty == "RSA" {
                let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)?;
                keys_map.insert(key.kid, decoding_key);
            }
        }

        self.jwks = Some(keys_map);
        Ok(())
    }

    // トークンから検証キーIDを取得
    fn get_kid_from_token(&self, token: &str) -> Result<String> {
        let header = decode_header(token)?;
        header
            .kid
            .ok_or_else(|| format_err!("Token header does not contain 'kid'"))
    }

    fn get_alg_from_token(&self, token: &str) -> Result<Algorithm> {
        let header = decode_header(token)?;
        Ok(header.alg)
        // .ok_or_else(|| format_err!("Token header does not contain 'alg'"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CognitoClaims {
    sub: String,
    iss: String,
    #[serde(rename = "cognito:groups")]
    groups: Option<Vec<String>>,
    #[serde(rename = "cognito:username")]
    username: Option<String>,
    #[serde(rename = "email")]
    email: Option<String>,
    exp: u64,
    iat: u64,
    aud: String,
    token_use: String,
    #[serde(rename = "scope")]
    scope: Option<String>,
}

#[async_trait]
impl TokenVerifier for CognitoTokenVerifier {
    async fn verify_access_token(&self, token: &str) -> Result<TokenClaims> {
        // // 可変のインスタンスを作成
        // let mut verifier = self.clone();

        // // JWKSがなければ取得
        // if verifier.jwks.is_none() {
        //     verifier.fetch_jwks().await?;
        // }

        // let kid = verifier.get_kid_from_token(token)?;
        // let alg = verifier.get_alg_from_token(token)?;

        // // JWKSとキーの取得
        // let jwks = verifier
        //     .jwks
        //     .as_ref()
        //     .ok_or_else(|| format_err!("JWKS not available"))?;
        // let key = jwks
        //     .get(&kid)
        //     .ok_or_else(|| format_err!("Key not found for kid: {}", kid))?;

        // // トークン検証設定
        // let mut validation = Validation::new(alg);
        // validation.set_audience(&[&self.client_id]);
        // validation.set_issuer(&[&format!(
        //     "https://cognito-idp.{}.amazonaws.com/{}",
        //     self.region, self.user_pool_id
        // )]);

        // // トークン検証
        // let token_data = decode::<CognitoClaims>(token, key, &validation)?;
        // let claims = token_data.claims;

        // // アクセストークン用途チェック
        // if claims.token_use != "access" {
        //     return Err(format_err!(
        //         "Invalid token use. Expected 'access' but got '{}'",
        //         claims.token_use
        //     ));
        // }

        // // ドメインクレームに変換
        // Ok(TokenClaims {
        //     sub: claims.sub,
        //     email: claims.email,
        //     groups: claims.groups.unwrap_or_default(),
        //     scope: claims.scope,
        //     exp: claims.exp,
        // })
        let keyset = KeySet::new(&self.region, &self.user_pool_id).unwrap();
        let verifier = keyset
            .new_access_token_verifier(&[&self.client_id])
            .build()?;

        let claims = keyset.verify(&token, &verifier).await.unwrap();
        // FIXEME: TokenClaims is not implemented yet
        Ok(TokenClaims {
            sub: "7754aa18-30f1-70b4-c147-821e623ff2c3".to_string(),
            email: None,
            groups: Vec::new(),
            scope: None,
            exp: 0,
        })
    }
}
