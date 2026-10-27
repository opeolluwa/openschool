package repositories

import (
	"github.com/google/uuid"
	"github.com/opeolluwa/openschool/plugins/region/models"
	"gorm.io/gorm"
)

type CountryRepository struct {
	db *gorm.DB
}

func NewCountryRepository(db *gorm.DB) CountryRepository {
	return CountryRepository{db}
}

func (r *CountryRepository) CreateCountry(country models.Country) error {
	result := r.db.Create(&country)
	return result.Error
}

func (r *CountryRepository) GetAll(page, limit int, search string) ([]models.Country, int64, error) {
	var countries []models.Country
	var total int64

	query := r.db.Model(&models.Country{})

	if search != "" {
		searchQuery := "%" + search + "%"
		query = query.Where("name ILIKE ? OR code ILIKE ? OR currency ILIKE ? OR curr ILIKE ?", searchQuery, searchQuery, searchQuery, searchQuery)
	}

	if err := query.Count(&total).Error; err != nil {
		return nil, 0, err
	}

	if page < 1 {
		page = 1
	}
	if limit < 1 {
		limit = 10
	}
	if limit > 100 {
		limit = 100
	}

	offset := (page - 1) * limit
	result := query.Offset(offset).Limit(limit).Find(&countries)
	if result.Error != nil {
		return nil, 0, result.Error
	}

	return countries, total, nil
}

func (r *CountryRepository) GetByIdentifier(identifier uuid.UUID) (*models.Country, error) {
	var country models.Country
	result := r.db.Where("identifier = ?", identifier).First(&country)
	if result.Error != nil {
		return nil, result.Error
	}
	return &country, nil
}

func (r *CountryRepository) GetByCode(code string) (*models.Country, error) {
	var country models.Country
	result := r.db.Where("code = ?", code).First(&country)
	if result.Error != nil {
		return nil, result.Error
	}
	return &country, nil
}

func (r *CountryRepository) Count() (int64, error) {
	var count int64
	err := r.db.Model(&models.Country{}).Count(&count).Error
	return count, err
}
