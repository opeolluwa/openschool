package middleware

import (
	"net/http"

	"github.com/go-playground/validator"
	"github.com/labstack/echo/v4"
)

type RequestValidator struct {
	Validator *validator.Validate
}

func (rv *RequestValidator) Validate(input any) error {
	if err := rv.Validator.Struct(input); err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}
	return nil
}

func NewRequestValidator() *RequestValidator {
	return &RequestValidator{Validator: validator.New()}
}
