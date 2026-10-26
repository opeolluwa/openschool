package requests

type CreateUserRequest struct {
	Email    string `json:"email" validate:"email"`
	Password string `json:"password" validate:"required"`
	FistName string `json:"firstName" validate:"required"`
	LastName string `json:"lastName" validate:"required"`
}
