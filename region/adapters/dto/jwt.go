package dto

import (
	"os"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

type UserClaims struct {
	Identifier string `json:"identifier"`
	Email      string `json:"email"`
	jwt.RegisteredClaims
}

func NewUserClaims(identifier, email string, expiry time.Duration) *UserClaims {
	return &UserClaims{
		Identifier: identifier,
		Email:      email,
		RegisteredClaims: jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(time.Now().Add(expiry)),
		},
	}
}

func (c *UserClaims) ToToken() (string, error) {
	jwtSigningKey := os.Getenv("JWT_SIGNING_KEY")
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, c)
	return token.SignedString([]byte(jwtSigningKey))
}
