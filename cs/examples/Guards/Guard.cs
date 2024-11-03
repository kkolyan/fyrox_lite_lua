using System;
using System.Collections.Generic;
using FyroxLite;

[Uuid("9f8183d3-2a4a-4951-a6e6-5fbc9c479e2e")]
public class Guard : NodeScript
{
    [HideInInspector]
    [Transient]
    private float ReloadingSeconds;
    
    private float ReloadDelaySeconds;
    private float GunHeight;
    private float SwitchWaypointTimeoutSeconds;
    
    [HideInInspector]
    [Transient]
    private float WaypointSeconds;
    
    [HideInInspector]
    [Transient]
    private Vector3? CurrentWaypoint;
    
    [HideInInspector]
    [Transient]
    private Node Collider;
    
    private Prefab BulletPrefab;
    private float InitialBulletVelocity;
    private float AttackRange;
    private float BeaconReachedDistance;
    private float MovePower;
    
    [HideInInspector]
    [Transient]
    private int Id;

    private const int FRACTION_GUARDS = 1;

    public void Init(int id)
    {
        Id = id;
    }

    public bool TryAttackPlayer()
    {
        Vector3 playerPos = Plugin.Get<Game>().Player.GlobalPosition;
        Vector3 selfPos = Node.GlobalPosition;
        Vector3 sightVector = playerPos - selfPos;

        if (CanSeePlayer(playerPos, sightVector))
        {
            Bullet.Spawn(new Bullet.BulletSeed
            {
                Prefab = BulletPrefab,
                Origin = selfPos + new Vector3(0.0f, GunHeight, 0.0f),
                Direction = sightVector,
                InitialVelocity = InitialBulletVelocity,
                AuthorCollider = Collider,
                Range = AttackRange,
                Fraction = FRACTION_GUARDS
            });
            ReloadingSeconds = ReloadDelaySeconds;
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
            if (node != Collider)
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
        WaypointSeconds += dt;
        if (WaypointSeconds > SwitchWaypointTimeoutSeconds)
        {
            CurrentWaypoint = null;
            WaypointSeconds = 0.0f;
        }

        if (CurrentWaypoint == null)
        {
            var beacons = Plugin.Get<Game>().Beacons;
            CurrentWaypoint = beacons[new Random().Next(beacons.Count)];
        }

        Vector3 pos = Node.LocalPosition;
        Vector3 vectorToBeacon = CurrentWaypoint.Value - pos;
        if (vectorToBeacon.Length() < BeaconReachedDistance)
        {
            CurrentWaypoint = null;
        }
        else
        {
            Vector3 force = vectorToBeacon.Normalized() * MovePower;
            Node.AsRigidBody().Value.ApplyForce(force);
        }
    }

    protected override void OnInit()
    {
        Collider = Node.FindColliderInChildren() ?? throw new Exception("Collider not found under Guard node");
    }

    protected override void OnStart()
    {
        Node.SubscribeTo();
    }

    protected override void OnUpdate(float dt)
    {
        if (ReloadingSeconds > 0.0f)
        {
            ReloadingSeconds -= dt;
        }

        if (ReloadingSeconds > 0.0f || !TryAttackPlayer())
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