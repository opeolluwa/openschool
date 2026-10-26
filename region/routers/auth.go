package routers

import (
	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/saturn/controllers"
	"github.com/opeolluwa/saturn/services"
	"github.com/opeolluwa/saturn/states"
)

func authRoutes(e *echo.Echo, state *states.State) *echo.Echo {

	authService := services.NewAuthService(state.Database)
	authController := controllers.NewAuthController(authService)

	authRoutes := e.Group("/auth")
	authRoutes.POST("/signup", authController.SignUp)
	authRoutes.POST("/login", authController.Login)

	return e
}
