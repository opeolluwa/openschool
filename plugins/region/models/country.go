package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type Country struct {
	Identifier uuid.UUID `gorm:"type:uuid;primaryKey" json:"identifier"`
	Name        string    `json:"name"`
	Code        string    `json:"code" gorm:"uniqueIndex"`
	Currency    string    `json:"currency"`
	Prefix      string    `json:"prefix"`
	CreatedAt   time.Time
	UpdatedAt   time.Time
	DeletedAt   gorm.DeletedAt `gorm:"index"`
}
