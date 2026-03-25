use anyhow::{anyhow, Result as AnyResult};
pub fn check_delimiter(delimiter: char) -> AnyResult<()> {
	if !delimiter.is_ascii()
		|| delimiter == '\r'
		|| delimiter == '\n'
		|| delimiter == '"'
		|| delimiter == '\0'
	{
		Err(anyhow!(
			"delimiter must be ascii and not '\\r', '\\n' or '\"'"
		))
	} else {
		Ok(())
	}
}

#[cfg(test)]
mod tests {}
