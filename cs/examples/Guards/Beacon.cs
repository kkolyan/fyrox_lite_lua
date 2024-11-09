using FyroxLite;

[Uuid("7c259fd2-fdb9-453b-a7ef-19cdd85428cc")]
public class Beacon : NodeScript
{
    protected override void OnUpdate(float dt)
    {
        Plugin.Get<Game>().beacons.Add(Node.GlobalPosition);

        Log.Info($"Beacon registered: {Node}");

        Node.Destroy();
    }
}