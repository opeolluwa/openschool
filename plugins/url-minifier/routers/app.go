package routers

import (
	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/openschool/plugins/url-minifier/states"
)

func LoadRoutes(e *echo.Echo, state *states.State) *echo.Echo {

	rootRouter(e)

	return e
}
