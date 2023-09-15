from nautilus_trader.adapters.binance.common.enums import BinanceAccountType
from nautilus_trader.adapters.binance.config import BinanceDataClientConfig
from nautilus_trader.adapters.binance.config import BinanceExecClientConfig
from nautilus_trader.adapters.binance.factories import BinanceLiveDataClientFactory
from nautilus_trader.adapters.binance.factories import BinanceLiveExecClientFactory
from nautilus_trader.common import Environment
from nautilus_trader.config import CacheDatabaseConfig
from nautilus_trader.config import InstrumentProviderConfig
from nautilus_trader.config import LiveExecEngineConfig
from nautilus_trader.config import LoggingConfig
from nautilus_trader.config import TradingNodeConfig
from nautilus_trader.examples.strategies.grid import GridConfig
from nautilus_trader.examples.strategies.grid import GridStrategy
from nautilus_trader.live.node import TradingNode


binance_api_key = "add19b12be488c715a2f6392dfac3f1c924c4ce2d2f56c1a3685ad9bb9f9f793"
binance_api_secret = "cc188431e33408d5fa62b4c22c4245126d50bab0c2139ee3693a4335b6787546"

config_node = TradingNodeConfig(
    trader_id="FILIP-001",
    environment=Environment.LIVE,
    logging=LoggingConfig(log_level="INFO"),
    exec_engine=LiveExecEngineConfig(
        reconciliation=True,
        reconciliation_lookback_mins=1440,
    ),
    cache_database=CacheDatabaseConfig(type="redis"),
    data_clients={
        "BINANCE": BinanceDataClientConfig(
            api_key=binance_api_key,
            api_secret=binance_api_secret,
            account_type=BinanceAccountType.USDT_FUTURE,
            testnet=True,
            instrument_provider=InstrumentProviderConfig(load_all=True),
        ),
    },
    exec_clients={
        "BINANCE": BinanceExecClientConfig(
            api_key=binance_api_key,
            api_secret=binance_api_secret,
            account_type=BinanceAccountType.SPOT,
            testnet=True,  # If client uses the testnet
            instrument_provider=InstrumentProviderConfig(load_all=True),
        ),
    },
    timeout_connection=20.0,
    timeout_reconciliation=10.0,
    timeout_portfolio=10.0,
    timeout_disconnection=10.0,
    timeout_post_stop=5.0,
)

node = TradingNode(config=config_node)

grid_config = GridConfig(
    instrument_id="ETHUSDT-PERP.BINANCE",
    value="grid",
)
grid_strategy = GridStrategy(config=grid_config)

node.trader.add_strategy(grid_strategy)
node.add_data_client_factory("BINANCE", BinanceLiveDataClientFactory)
node.add_exec_client_factory("BINANCE", BinanceLiveExecClientFactory)
node.build()

if __name__ == "__main__":
    try:
        node.run()
    finally:
        node.dispose()
