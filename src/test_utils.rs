pub fn test_assert(value: bool, message: String) {
	if !value {
		panic!(message);
	}
}
