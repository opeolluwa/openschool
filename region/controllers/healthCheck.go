package controllers

import (
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/saturn/adapters/responses"
)

func HealthCheck(c echo.Context) error {
	return c.JSON(http.StatusOK, responses.New("service is healthy"))
}
