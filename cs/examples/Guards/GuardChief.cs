using FyroxLite;

[Uuid("c69ae5fa-de26-4ee5-b70c-113df285f6e2")]
public class GuardChief : NodeScript
{
    private Prefab gaurd_prefab;
    private float initial_count;

    [HideInInspector] [Transient] private bool initialized;

    [HideInInspector] [Transient] private bool frame_skipped_for_beacons;

    protected override void OnUpdate(float dt)
    {
        if (!frame_skipped_for_beacons)
        {
            frame_skipped_for_beacons = true;
            return;
        }

        if (!initialized)
        {
            initialized = true;
            for (int i = 1; i <= initial_count; i++)
            {
                var beacons = Plugin.Get<Game>().beacons;
                if (beacons.Count > 0)
                {
                    var position = beacons[new Random().Next(beacons.Count)];
                    var angle = (float)(new Random().NextDouble() * 2 * Math.PI);

                    var guard = gaurd_prefab.InstantiateAt(position, new Quaternion(Vector3.Up, angle));
                    guard.FindScript<Guard>().Init(i);

                    Log.Info($"Guard spawned at {position}");
                }
                else
                {
                    Log.Err("Cannot spawn guards: no beacons found");
                }
            }
        }
    }
}