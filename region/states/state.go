package states

import (
	"log"

	"github.com/opeolluwa/saturn/config"
	"gorm.io/gorm"
)

type State struct {
	Database *gorm.DB
}

func Init(environment config.Environment) (State, error) {

	db, err := config.ConnectDatabase(environment)
	if err != nil {
		log.Println(err)
		return State{}, err
	}

	return State{
		Database: db,
	}, nil
}
