use super::primitives::PRIMITIVE_TOKENS;
use super::semantics::SEMANTIC_TOKENS;
use super::families::*;

pub const PUBLIC_API_TOKENS: &[&[&str]] = &[
    FAMILY_A_TOKENS, FAMILY_B_TOKENS, FAMILY_C_TOKENS,
    FAMILY_D_TOKENS, FAMILY_E_TOKENS, FAMILY_F_TOKENS,
    FAMILY_G_TOKENS,
    FAMILY_H_TOKENS,
];

pub const ALL_TOKENS: &[&[&str]] = &[
    PRIMITIVE_TOKENS, SEMANTIC_TOKENS,
    FAMILY_A_TOKENS, FAMILY_B_TOKENS, FAMILY_C_TOKENS,
    FAMILY_D_TOKENS, FAMILY_E_TOKENS, FAMILY_F_TOKENS,
    FAMILY_G_TOKENS,
    FAMILY_H_TOKENS,
];

pub fn token_exists(token: &str) -> bool {
    ALL_TOKENS.iter().flat_map(|g| g.iter()).any(|t| *t == token)
}

pub fn is_public_api_token(token: &str) -> bool {
    PUBLIC_API_TOKENS.iter().flat_map(|g| g.iter()).any(|t| *t == token)
}
