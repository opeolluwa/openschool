package responses

type ApiResponse struct {
	Message string `json:"message"`
}

func Error(message string) ApiResponse {
	return ApiResponse{Message: message}
}



func New(message string) ApiResponse {
	return ApiResponse{Message: message}
}
