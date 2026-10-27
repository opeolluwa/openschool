package routers

import (
	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/openschool/plugins/url-minifier/controllers"
)

func rootRouter(e *echo.Echo) {
	e.GET("/health", controllers.HealthCheck)
}
