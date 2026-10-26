package services

import (
	"fmt"
	"net/http"
	"time"

	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/saturn/adapters/dto"
	"github.com/opeolluwa/saturn/adapters/requests"
	"github.com/opeolluwa/saturn/adapters/responses"
	"github.com/opeolluwa/saturn/repositories"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

type AuthService struct {
	userRepository repositories.UserRepository
}

func NewAuthService(db *gorm.DB) AuthService {
	userRepository := repositories.NewUserRepository(db)
	return AuthService{userRepository}
}

func (s *AuthService) CreateUser(ctx echo.Context, req requests.CreateUserRequest) error {

	err := s.userRepository.CreateUser(req)
	if err != nil {
		return ctx.String(http.StatusInternalServerError, err.Error())
	}

	return nil
}

func (s *AuthService) Login(ctx echo.Context, req requests.LoginRequest) (responses.LoginResponse, error) {
	user, err := s.userRepository.FindByEmail(req.Email)

	if err != nil {
		return responses.LoginResponse{}, err
	}

	if !verifyPassword(user.Password, req.Password) {
		return responses.LoginResponse{}, echo.ErrUnauthorized
	}

	claims := dto.NewUserClaims(user.Identifier.String(), user.Email, time.Duration(time.Minute*5))

	token, err := claims.ToToken()
	if err != nil {
		fmt.Println(err)
		return responses.LoginResponse{}, echo.ErrInternalServerError
	}

	return responses.LoginResponse{Token: token}, nil

}

func verifyPassword(hashedPassword, password string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(password))
	return err == nil
}
