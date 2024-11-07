using Exception = System.Exception;

namespace FyroxLite;

public abstract class NodeScript
{
    [HideInInspector]
    [Transient]
    internal Node _node;
    public ref Node Node => ref _node;

    protected internal virtual void OnInit()
    {
    }

    protected internal virtual void OnStart()
    {
    }

    protected internal virtual void OnUpdate(float dt)
    {
    }

    protected internal virtual void OnMessage(object message)
    {
    }

    protected internal virtual void OnDeinit()
    {
    }
}