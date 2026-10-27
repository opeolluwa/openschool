package routers

import (
	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/openschool/plugins/region/states"
)

func LoadRoutes(e *echo.Echo, state *states.State) *echo.Echo {

	countryRoutes(e, state)
	rootRouter(e)

	return e
}
