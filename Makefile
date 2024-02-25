DB_URL := sqlite://files.sqlite

orm-migration-refresh:
	sea-orm-cli migrate refresh -u $(DB_URL)
