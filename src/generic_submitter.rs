use bitcoin::blockdata::block::BlockHeader;

// Plug in your business logic into these functions:

pub struct GenericSubmitterSettings { }
pub struct GenericSubmitterState { }

pub fn init_submitter_settings() -> GenericSubmitterSettings {
	GenericSubmitterSettings { }
}

pub fn print_submitter_parameters() { }

/// Returns true if the given parameter could be parsed into a setting this submitter understands
pub fn parse_submitter_parameter(_settings: &mut GenericSubmitterSettings, _arg: &str) -> bool {
	// we have no settings for a submitter that just prints...
	false
}

pub fn setup_submitter(_settings: GenericSubmitterSettings) -> GenericSubmitterState {
	GenericSubmitterState { }
}

/// Returns true if the given user_id/auth pair is valid for this pool. Note that the pool_proxy
/// stuff doesn't really bother with auth, so if you use it you probably can't reliably check
/// user_auth, but there probably isnt any reason to ever anyway...
pub fn check_user_auth(_state: &GenericSubmitterState, user_id: &Vec<u8>, user_auth: &Vec<u8>) -> bool {
	println!("User {} authed with pass {}", String::from_utf8_lossy(user_id), String::from_utf8_lossy(user_auth));
	true
}

pub fn share_submitted(_state: &GenericSubmitterState, user_id: &Vec<u8>, user_tag_1: &Vec<u8>, value: u64, _header: &BlockHeader, _leading_zeros: u8, _required_leading_zeros: u8) {
	println!("Got valid share with value {} from \"{}\" from machine identified as \"{}\"", value, String::from_utf8_lossy(user_id), String::from_utf8_lossy(user_tag_1));
}

pub fn weak_block_submitted(_state: &GenericSubmitterState, user_id: &Vec<u8>, user_tag_1: &Vec<u8>, value: u64, _header: &BlockHeader, txn: &Vec<Vec<u8>>, _extra_block_data: &Vec<u8>, _leading_zeros: u8, _required_leading_zeros: u8) {
	println!("Got valid weak block with value {} from \"{}\" with {} txn from machine identified as \"{}\"", value, String::from_utf8_lossy(user_id), txn.len(), String::from_utf8_lossy(user_tag_1));
}