#pragma once
#include "stdafx.h"
#include <exception>

class GameException : std::exception
{
private:
	const string _what;

public:
	GameException(const string& what);
	~GameException();

	const char* what() const;
};
