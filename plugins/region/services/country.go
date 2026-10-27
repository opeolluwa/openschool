package services

import (
	"math"

	"github.com/google/uuid"
	"github.com/opeolluwa/openschool/plugins/region/adapters"
	"github.com/opeolluwa/openschool/plugins/region/repositories"
	"gorm.io/gorm"
)

type CountryService struct {
	countryRepository repositories.CountryRepository
}

func NewCountryService(db *gorm.DB) CountryService {
	countryRepository := repositories.NewCountryRepository(db)
	return CountryService{countryRepository}
}

func (s *CountryService) GetAll(page, limit int, search string) (adapters.CountriesListResponse, error) {
	countries, total, err := s.countryRepository.GetAll(page, limit, search)
	if err != nil {
		return adapters.CountriesListResponse{}, err
	}

	if page < 1 {
		page = 1
	}
	if limit < 1 {
		limit = 10
	}

	countryResponses := make([]adapters.CountryResponse, 0, len(countries))
	for _, country := range countries {
		countryResponses = append(countryResponses, adapters.CountryResponse{
			Identifier: country.Identifier,
			Name:        country.Name,
			Code:        country.Code,
			Currency:    country.Currency,
			Prefix:      country.Prefix,
		})
	}

	totalPages := int(math.Ceil(float64(total) / float64(limit)))
	if totalPages == 0 {
		totalPages = 0
	}

	return adapters.CountriesListResponse{
		Countries:  countryResponses,
		Total:      total,
		Page:       page,
		Limit:      limit,
		TotalPages: totalPages,
	}, nil
}

func (s *CountryService) GetByIdentifier(identifier uuid.UUID) (adapters.CountryResponse, error) {
	country, err := s.countryRepository.GetByIdentifier(identifier)
	if err != nil {
		return adapters.CountryResponse{}, err
	}

	return adapters.CountryResponse{
		Identifier: country.Identifier,
		Name:        country.Name,
		Code:        country.Code,
		Currency:    country.Currency,
		Prefix:      country.Prefix,
	}, nil
}
