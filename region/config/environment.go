package config

type Environment struct {
	Server struct {
		Port        string `env:"PORT,default=5008"`
		Environment string `env:"ENVIRONMENT,default=development"`
	}
	Database struct {
		Host     string `env:"DATABASE_HOST,required=true"`
		Port     string `env:"DATABASE_PORT,required=true"`
		Username string `env:"DATABASE_USER,required=true"`
		Password string `env:"DATABASE_PASSWORD,required=true"`
		Name     string `env:"DATABASE_NAME,required=true"`
	}
}
