package controllers

import (
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/openschool/plugins/region/adapters"
)

func HealthCheck(c echo.Context) error {
	return c.JSON(http.StatusOK, adapters.New("service is healthy"))
}
