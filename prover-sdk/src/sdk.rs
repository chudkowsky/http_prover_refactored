use common::ProverInput;
use reqwest::{Client, Response};
use url::Url;

use crate::errors::SdkErrors;
#[derive(Debug, Clone)]
/// ProverSDK is a struct representing a client for interacting with the Prover service.
pub struct ProverSDK {
    pub client: Client,
    pub prover_cairo0: Url,
    pub prover_cairo: Url,
    pub verify: Url,
    pub get_job: Url,
}

impl ProverSDK {
    pub fn new(url: Url) -> Result<Self, SdkErrors> {
        let client = Client::new();
        let prover_cairo0 = url.join("/prove/cairo0")?;
        let prover_cairo = url.join("/prove/cairo")?;
        let verify = url.join("/verify")?;
        let get_job = url.join("/get-job")?;
        Ok(Self {
            client,
            prover_cairo0,
            prover_cairo,
            verify,
            get_job,

        })
    }

    pub async fn prove_cairo0<T>(&self, data: T) -> Result<String, SdkErrors>
    where
        T: ProverInput + Send + 'static,
    {
        self.prove(data, self.prover_cairo0.clone()).await
    }

    pub async fn prove_cairo<T>(&self, data: T) -> Result<String, SdkErrors>
    where
        T: ProverInput + Send + 'static,
    {
        self.prove(data, self.prover_cairo.clone()).await
    }

    async fn prove<T>(&self, data: T, url: Url) -> Result<String, SdkErrors>
    where
        T: ProverInput + Send + 'static,
    {
        let response = self
            .client
            .post(url.clone())
            .json(&data.serialize())
            .send()
            .await?;

        if !response.status().is_success() {
            let response_data: String = response.text().await?;
            tracing::error!("{}", response_data);
            return Err(SdkErrors::ProveResponseError(response_data));
        }
        let response_data = response.text().await?;

        Ok(response_data)
    }
    pub async fn verify(self, proof: String) -> Result<String, SdkErrors> {
        let response = self
            .client
            .post(self.verify.clone())
            .json(&proof)
            .send()
            .await?;
        let response_data = response.text().await?;
        Ok(response_data)
    }
    pub async fn get_job(&self, job_id: u64) -> Result<Response, SdkErrors> {
        // Construct the URL with the job_id
        let url = format!("{}/{}", self.get_job.clone().as_str(), job_id);
        // Send the GET request to the constructed URL
        let response = self.client.get(url).send().await?;

        // Check if the response status is successful
        if !response.status().is_success() {
            let response_data: String = response.text().await?;
            tracing::error!("{}", response_data);
            return Err(SdkErrors::GetJobResponseError(response_data));
        }
        // Parse the response data
        Ok(response)
    }
}
