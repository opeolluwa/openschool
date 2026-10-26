package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type User struct {
	Identifier uuid.UUID `gorm:"type:uuid;primaryKey" json:"identifier"`
	Email      string    `json:"email"`
	Password   string
	FirstName  string `json:"firstName"`
	LastName   string `json:"lastName"`
	CreatedAt  time.Time
	UpdatedAt  time.Time
	DeletedAt  gorm.DeletedAt `gorm:"index"`
}
