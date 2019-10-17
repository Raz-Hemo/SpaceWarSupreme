#include "stdafx.h"
#include "GameException.h"

void FatalError(const string& error)
{
	throw GameException(error);
}

bool isDebug()
{
#ifdef _DEBUG
	return true;
#else
	return false;
#endif
}