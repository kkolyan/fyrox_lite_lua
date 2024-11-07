using System;
using System.Collections.Generic;
using FyroxLite;

[Uuid("9f8183d3-2a4a-4951-a6e6-5fbc9c479e2e")]
public class Guard : NodeScript
{
    [HideInInspector]
    [Transient]
    private float reloading_sec;
    
    private float reload_delay_sec;
    private float gun_height;
    private float switch_waypoint_timeout_sec;
    
    [HideInInspector]
    [Transient]
    private float waypoint_sec;
    
    [HideInInspector]
    [Transient]
    private Vector3? current_waypoint;
    
    [HideInInspector]
    [Transient]
    private Node collider;
    
    private Prefab bullet_prefab;
    private float initial_bullet_velocity;
    private float attack_range;
    private float beacon_reached_distance;
    private float move_power;
    
    [HideInInspector]
    [Transient]
    private int id;

    private const int FRACTION_GUARDS = 1;

    public void Init(int id)
    {
        this.id = id;
    }

    public bool TryAttackPlayer()
    {
        Vector3 playerPos = Plugin.Get<Game>().player.GlobalPosition;
        Vector3 selfPos = Node.GlobalPosition;
        Vector3 sightVector = playerPos - selfPos;

        if (CanSeePlayer(playerPos, sightVector))
        {
            Bullet.Spawn(new Bullet.BulletSeed
            {
                Prefab = bullet_prefab,
                Origin = selfPos + new Vector3(0.0f, gun_height, 0.0f),
                Direction = sightVector,
                InitialVelocity = initial_bullet_velocity,
                AuthorCollider = collider,
                Range = attack_range,
                Fraction = FRACTION_GUARDS
            });
            reloading_sec = reload_delay_sec;
            return true;
        }

        return false;
    }

    public bool CanSeePlayer(Vector3 playerPos, Vector3 sightVector)
    {
        RayCastOptions opts = new RayCastOptions
        {
            RayOrigin = playerPos,
            RayDirection = sightVector.Normalized(),
            MaxLen = sightVector.Length(),
            SortResults = true
        };

        List<Intersection> results = Physics.CastRay(opts);
        foreach (var hit in results)
        {
            Node node = hit.Collider;
            if (node != collider)
            {
                while (node.Alive)
                {
                    if (node.FindScript<Player>() != null)
                    {
                        return true;
                    }

                    node = node.Parent;
                }

                return false;
            }
        }

        return false;
    }

    public void MoveToWaypoint(float dt)
    {
        waypoint_sec += dt;
        if (waypoint_sec > switch_waypoint_timeout_sec)
        {
            current_waypoint = null;
            waypoint_sec = 0.0f;
        }

        if (current_waypoint == null)
        {
            var beacons = Plugin.Get<Game>().beacons;
            current_waypoint = beacons[new Random().Next(beacons.Count)];
        }

        Vector3 pos = Node.LocalPosition;
        Vector3 vectorToBeacon = current_waypoint.Value - pos;
        if (vectorToBeacon.Length() < beacon_reached_distance)
        {
            current_waypoint = null;
        }
        else
        {
            Vector3 force = vectorToBeacon.Normalized() * move_power;
            Node.AsRigidBody().Value.ApplyForce(force);
        }
    }

    protected override void OnInit()
    {
        collider = Node.FindColliderInChildren() ?? throw new Exception("Collider not found under Guard node");
    }

    protected override void OnStart()
    {
        Node.SubscribeTo();
    }

    protected override void OnUpdate(float dt)
    {
        if (reloading_sec > 0.0f)
        {
            reloading_sec -= dt;
        }

        if (reloading_sec > 0.0f || !TryAttackPlayer())
        {
            MoveToWaypoint(dt);
        }
    }

    public void OnMessage(object message)
    {
        if (message is BulletHitMessage hit && hit.Fraction != FRACTION_GUARDS)
        {
            Node.Destroy();
            Plugin.Get<Game>().IncFrags();
            Console.WriteLine("Guard killed!");
        }
    }
}
