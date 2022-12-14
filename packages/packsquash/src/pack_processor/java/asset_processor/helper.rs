pub(super) mod json_helper;

macro_rules! get_file_specific_options {
	($file_options_map:expr , $asset_path:expr , $file_specific_options_type:ident) => {
		::once_cell::unsync::Lazy::new(|| {
			$file_options_map
				.get($asset_path)
				.find_map(|file_options| {
					if let ::packsquash_options::FileOptions::$file_specific_options_type(
						file_options
					) = file_options
					{
						Some(file_options)
					} else {
						None
					}
				})
				.unwrap_or_default()
		})
	};
}
