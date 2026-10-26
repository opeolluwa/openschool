package routers

import (
	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/saturn/states"
)

func LoadRoutes(e *echo.Echo, state *states.State) *echo.Echo {
	
	authRoutes(e, state)
	rootRouter(e)
	
	return e
}
