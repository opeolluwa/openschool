package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/Netflix/go-env"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/opeolluwa/openschool/plugins/region/config"
	appMiddleware "github.com/opeolluwa/openschool/plugins/region/middlewares"
	"github.com/opeolluwa/openschool/plugins/region/routers"
	"github.com/opeolluwa/openschool/plugins/region/seeders"
	"github.com/opeolluwa/openschool/plugins/region/states"
	"github.com/swaggo/echo-swagger"
)

func main() {

	var environment config.Environment

	_, err := env.UnmarshalFromEnviron(&environment)
	if err != nil {
		log.Fatalln(err)
	}

	state, err := states.Init(environment)
	if err != nil {
		log.Fatalln(err)
	}

	if err := seeders.SeedCountries(state.Database); err != nil {
		log.Println("Warning: failed to seed countries:", err)
	}

	app := echo.New()

	app.GET("/swagger/*", echoSwagger.WrapHandlerV3)

	app.Use(middleware.RequestLogger())
	app.Use(middleware.Recover())
	app.Use(middleware.CORS())
	app.Use(middleware.Gzip())

	app.Use(middleware.RateLimiterWithConfig(config.RateLimitConfig()))
	app.Validator = appMiddleware.NewRequestValidator()

	routers.LoadRoutes(app, &state)
	port := environment.Server.Port

	go func() {
		if err := app.Start(fmt.Sprintf(":%s", port)); err != nil && err != http.ErrServerClosed {
			app.Logger.Fatal(err)
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, os.Interrupt, syscall.SIGTERM)
	<-quit

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	if err := app.Shutdown(ctx); err != nil {
		app.Logger.Fatal(err)
	}
}
