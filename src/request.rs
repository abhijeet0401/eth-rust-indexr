use crate::Validator;
use serde_json::Value;
pub async fn get_propser(epoch: &i32) -> std::io::Result<usize> {
    let string_epoch = epoch.to_string();

    let url = format!(
        "https://docs-demo.quiknode.pro/eth/v1/validator/duties/proposer/{}",
        string_epoch
    );

    let body = reqwest::get(url)
        .await
        .expect("not able to make get request")
        .text()
        .await;
    let json: Value = serde_json::from_str(body.as_ref().unwrap())?;
    let data = &json["data"];
    let validator_length = data.as_array().expect("not able to fetch array").len();
    Ok(validator_length)
}
pub fn calculate_result(latest_epoch: &i32, validator_length: &i32) -> Validator {
    let slot_size = 32;
    let k = (slot_size - validator_length) / latest_epoch * slot_size * validator_length;
    let np = 1 - k;
    let vp = 1 - k * validator_length;
    let mut index_result: Validator = Validator::new();
    index_result.epoch = *latest_epoch;
    index_result.network_participation = np;
    index_result.validator_participation = vp;
    index_result
}
pub async fn get_finalized_epoch() -> std::io::Result<i32> {
    let body = reqwest::get(
        "https://docs-demo.quiknode.pro/eth/v1/beacon/states/justified/finality_checkpoints",
    )
    .await
    .expect("not able to make get request")
    .text()
    .await;
    let json: Value = serde_json::from_str(body.as_ref().unwrap())?;
    let data = &json["data"];
    let finalized = &data["current_justified"];
    let finalized_epoch = &finalized["epoch"];
    let latest_epoch = finalized_epoch.as_str().unwrap().parse::<i32>().unwrap();
    println!("{}", latest_epoch);
    Ok(latest_epoch)
}
