#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EscapeOutcome {
	NotEscaped,
	DuplicatedQuote,
}