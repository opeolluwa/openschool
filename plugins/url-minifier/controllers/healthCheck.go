package controllers

import (
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/openschool/plugins/url-minifier/adapters"
)

func HealthCheck(c echo.Context) error {
	return c.JSON(http.StatusOK, adapters.New("service is healthy"))
}
