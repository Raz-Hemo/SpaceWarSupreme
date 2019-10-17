#include "stdafx.h"
#include "GameException.h"

GameException::GameException(const string& what) : _what(what)
{
}

GameException::~GameException()
{
}

const char* GameException::what() const
{
	return _what.c_str();
}