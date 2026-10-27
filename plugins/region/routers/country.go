package routers

import (
	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/openschool/plugins/region/controllers"
	"github.com/opeolluwa/openschool/plugins/region/services"
	"github.com/opeolluwa/openschool/plugins/region/states"
)

func countryRoutes(e *echo.Echo, state *states.State) *echo.Echo {
	countryService := services.NewCountryService(state.Database)
	countryController := controllers.NewCountryController(countryService)

	countryRoutes := e.Group("/countries")
	countryRoutes.GET("", countryController.GetAll)
	countryRoutes.GET("/:id", countryController.GetOne)

	return e
}
