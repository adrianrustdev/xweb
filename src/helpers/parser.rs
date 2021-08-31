use serde_json::Value;

pub fn get_id_from_json(data: String) -> Result<String, serde_json::Error> {
    let json_data: Value = serde_json::from_str(&*data)?;
    Ok(json_data["result"].to_string())
}

pub fn get_timestamp_from_json(data: String) -> Result<String, serde_json::Error> {
    let json_data: Value = serde_json::from_str(&*data)?;
    let json_data_details_string = json_data["result"].to_string();
    let json_data_details: Value = serde_json::from_str(&*json_data_details_string)?;
    Ok(json_data_details["timestamp"].to_string())
}