#include "stdafx.h"
#include "GameObject.h"
#include "Renderable.h"

GameObject::GameObject()
{
	DirectX::XMStoreFloat4x4(&transform, DirectX::XMMatrixIdentity());
	DirectX::XMStoreFloat3(&velocity, DirectX::XMVectorZero());
}

GameObject::~GameObject()
{

}

void GameObject::AddChild(shared_ptr<GameObject> child)
{
	children.push_back(child);
	child->parent.reset(this);
}

void GameObject::Detach(shared_ptr<GameObject> child)
{
	child->parent = nullptr;
	children.remove(child);
}

const list<shared_ptr<GameObject>>& GameObject::GetChildren() const
{
	return children;
}

void GameObject::SetRenderable(shared_ptr<Renderable> r)
{
	renderable = r;
}
