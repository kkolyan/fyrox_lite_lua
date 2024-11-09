using FyroxLite;

[Uuid("c5671d19-9f1a-4286-8486-add4ebaadaec")]
public class Player: NodeScript
{
    private float sensitivity;
    private Node camera;
    private float power;
    private Prefab bullet;
    private float initial_bullet_velocity;
    private float shooting_range;
    private float reload_delay_sec;
    
    [HideInInspector]
    [Transient]
    private float ReloadSec;
    
    [HideInInspector]
    [Transient]
    private bool Published;
    
    [HideInInspector]
    [Transient]
    private Node Collider;
    
    [HideInInspector]
    [Transient]
    private float AimY;

    public const int FractionPlayer = 0;

    public void Turn(float x)
    {
        Quaternion rotDelta = Quaternion.FromEuler(Vector3.Up * sensitivity * x);
        Node.LocalRotation *= rotDelta;
    }

    public void Aim(float y)
    {
        AimY += y * sensitivity;
        AimY = Math.Max(-MathF.PI / 2.0f, Math.Min(AimY, MathF.PI / 2.0f));

        camera.LocalRotation = Quaternion.FromEuler(Vector3.Left * AimY);
    }

    public void Fire()
    {
        Vector3 cameraPos = camera.GlobalPosition;
        Quaternion bulletOrientation = camera.GlobalRotation;

        Bullet.Spawn(new Bullet.BulletSeed
        {
            Prefab = bullet,
            Origin = cameraPos,
            Direction = bulletOrientation * Vector3.Forward,
            InitialVelocity = initial_bullet_velocity,
            AuthorCollider = Collider,
            Range = shooting_range,
            Fraction = FractionPlayer,
        });
    }

    protected override void OnInit()
    {
        Window.CursorGrab = CursorGrabMode.Confined;
        Collider = Node.FindColliderInChildren() ?? throw new Exception("player collider missing");
    }

    protected override void OnStart()
    {
        Node.SubscribeTo();
    }

    protected override void OnMessage(object message)
    {
        if (message is BulletHitMessage hitMessage && hitMessage.Fraction != FractionPlayer)
        {
            Plugin.Get<Game>().IncWounds();
            Console.WriteLine("player wounded!");
        }
    }

    protected override void OnUpdate(float dt)
    {
        if (ReloadSec > 0.0f)
        {
            ReloadSec -= dt;
        }
        if (!Published)
        {
            Published = true;
            Plugin.Get<Game>().player = Node;
        }

        if (Input.IsMouseButton(Input.MouseLeft))
        {
            if (ReloadSec <= 0.0f)
            {
                ReloadSec = reload_delay_sec;
                Fire();
            }
        }

        Vector3 moveDelta = Vector3.Zero;

        if (Input.IsKey(KeyCode.W))
        {
            moveDelta.Z += 1.0f;
        }
        if (Input.IsKey(KeyCode.S))
        {
            moveDelta.Z -= 1.0f;
        }
        if (Input.IsKey(KeyCode.A))
        {
            moveDelta.X += 1.0f;
        }
        if (Input.IsKey(KeyCode.D))
        {
            moveDelta.X -= 1.0f;
        }

        Turn(-Input.MouseMove.X);
        Aim(Input.MouseMove.Y);

        if (moveDelta.Length() > 0.001f)
        {
            moveDelta = moveDelta.Normalized();
        }

        var selfRotation = Node.LocalRotation;
        Vector3 moveDirection = selfRotation * moveDelta;
        Vector3 force = moveDirection * power;
        Node.AsRigidBody().Value.ApplyForce(force);
    }
}
