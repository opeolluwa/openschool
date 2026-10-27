package controllers

import (
	"net/http"

	"github.com/google/uuid"
	"github.com/labstack/echo/v4"
	"github.com/opeolluwa/openschool/plugins/region/adapters"
	"github.com/opeolluwa/openschool/plugins/region/services"
)

type CountryController struct {
	countryService services.CountryService
}

func NewCountryController(countryService services.CountryService) CountryController {
	return CountryController{countryService}
}

func (c *CountryController) GetAll(ctx echo.Context) error {
	var req adapters.GetCountriesRequest
	if err := ctx.Bind(&req); err != nil {
		req.Page = 1
		req.Limit = 10
	}

	if req.Page < 1 {
		req.Page = 1
	}
	if req.Limit < 1 {
		req.Limit = 10
	}

	result, err := c.countryService.GetAll(req.Page, req.Limit, req.Search)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, adapters.Error(err.Error()))
	}

	return ctx.JSON(http.StatusOK, result)
}

func (c *CountryController) GetOne(ctx echo.Context) error {
	id := ctx.Param("id")
	identifier, err := uuid.Parse(id)
	if err != nil {
		return ctx.JSON(http.StatusBadRequest, adapters.Error("invalid identifier"))
	}

	result, err := c.countryService.GetByIdentifier(identifier)
	if err != nil {
		return ctx.JSON(http.StatusNotFound, adapters.Error("country not found"))
	}

	return ctx.JSON(http.StatusOK, result)
}
