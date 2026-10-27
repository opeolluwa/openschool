package seeders

import (
	"encoding/json"
	"os"
	"path/filepath"

	"github.com/google/uuid"
	"github.com/opeolluwa/openschool/plugins/region/models"
	"gorm.io/gorm"
)

type SeedData struct {
	Countries []struct {
		Name     string `json:"name"`
		Code     string `json:"code"`
		Currency string `json:"currency"`
		Prefix   string `json:"prefix"`
	} `json:"countries"`
}

func SeedCountries(db *gorm.DB) error {
	var count int64
	if err := db.Model(&models.Country{}).Count(&count).Error; err != nil {
		return err
	}
	if count > 0 {
		return nil
	}

	// Try to find seed.json
	seedPaths := []string{
		"seed.json",
		"./seed.json",
		"../seed.json",
		filepath.Join(filepath.Dir(os.Args[0]), "seed.json"),
	}

	var seedFile []byte
	var err error
	for _, path := range seedPaths {
		seedFile, err = os.ReadFile(path)
		if err == nil {
			break
		}
	}
	if err != nil {
		return err
	}

	var seedData SeedData
	if err := json.Unmarshal(seedFile, &seedData); err != nil {
		return err
	}

	for _, c := range seedData.Countries {
		country := models.Country{
			Identifier: uuid.New(),
			Name:       c.Name,
			Code:       c.Code,
			Currency:   c.Currency,
			Prefix:     c.Prefix,
		}
		if err := db.Create(&country).Error; err != nil {
			continue
		}
	}

	return nil
}
