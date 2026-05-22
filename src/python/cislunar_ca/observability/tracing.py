from functools import wraps
from typing import Any, Callable
from ..config import settings


def setup_weave():
    if settings.weave_enabled:
        try:
            import weave
            weave.init(settings.weave_project)
            return True
        except ImportError:
            import warnings
            warnings.warn("W&B Weave not installed. Install with: pip install weave")
    return False


def trace_ca_operation(operation_name: str = None):
    def decorator(func: Callable) -> Callable:
        @wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> Any:
            if settings.weave_enabled:
                try:
                    import weave
                    op = weave.op(func)
                    return await op(*args, **kwargs)
                except ImportError:
                    return await func(*args, **kwargs)
            return await func(*args, **kwargs)
        return wrapper
    return decorator
