#include "stdafx.h"
#include "GameException.h"

GameException::GameException(const wstring& what) : _what(what)
{
}

GameException::~GameException()
{
}

const wstring& GameException::what() const
{
	return _what;
}