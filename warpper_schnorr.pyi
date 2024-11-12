from typing import Union


def private_key_to_public_key(private_key: str) -> str : ...

def generate_keys_with_seed(seed: str) -> Union[str, str] : ...


def verify_signature(
    message: str,
    signature: str,
    public_key: str,
    context: str,
) -> bool: ...


def sign_message(message: str, private_key: str, context: str) -> str : ...
