run-prod:
	cargo run --  -c configs/config-prod.yaml -m migrations/1.sql
run-dev-1:
	 cargo run --  -c configs/config-dev.yaml -m migrations/1.sql
run-dev-2:
	cargo run --  -c configs/config-dev.yaml -m migrations/2.sql		