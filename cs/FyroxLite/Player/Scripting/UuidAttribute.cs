namespace FyroxLite;

[AttributeUsage(AttributeTargets.Class)]
public class UuidAttribute : Attribute
{
    public readonly Guid Uuid;

    public UuidAttribute(string uuid)
    {
        Uuid = new Guid(uuid);
    }
}