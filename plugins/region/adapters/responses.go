package adapters

import (
	"github.com/google/uuid"
)

type ApiResponse struct {
	Message string `json:"message"`
}

func Error(message string) ApiResponse {
	return ApiResponse{Message: message}
}

func New(message string) ApiResponse {
	return ApiResponse{Message: message}
}

type CountryResponse struct {
	Identifier uuid.UUID `json:"identifier"`
	Name        string    `json:"name"`
	Code        string    `json:"code"`
	Currency    string    `json:"currency"`
	Prefix      string    `json:"prefix"`
}

type CountriesListResponse struct {
	Countries  []CountryResponse `json:"countries"`
	Total      int64             `json:"total"`
	Page       int               `json:"page"`
	Limit      int               `json:"limit"`
	TotalPages int               `json:"totalPages"`
}
