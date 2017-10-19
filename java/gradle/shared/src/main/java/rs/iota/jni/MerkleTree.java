package rs.iota.jni;

public class MerkleTree {
    private long INSTANCE_PTR;

    private long cachedSize = -1, cachedDepth = -1;
    private final int count;
    private int[] cachedSlice = null;

    public MerkleTree(int[] seed, int index, int count, int security) {
        if(seed == null || seed.length % 243 != 0) {
            throw new IllegalArgumentException("Invalid seed parameter provided.");
        }
        this.INSTANCE_PTR = nativeCreate(seed, index, count, security);
        this.count = 0;
    }


    public void delete() {
        if (INSTANCE_PTR != 0) {
            nativeDrop(INSTANCE_PTR);
            INSTANCE_PTR = 0;
        }
    }


    public MerkleBranch branch(int index) {
        return new MerkleBranch(nativeBranch(INSTANCE_PTR, index), index);
    }


    public int[] root() {
        return slice();
    }

    public int[] slice() {
        if (cachedSlice == null) {
            cachedSlice = nativeSlice(INSTANCE_PTR);
        }

        return cachedSlice.clone();
    }

    public int count() {
        return this.count;
    }

    public long size() {
        if (cachedSize == -1) {
            cachedSize = nativeSize(INSTANCE_PTR);
        }

        return cachedSize;
    }

    public long depth() {
        if (cachedDepth == -1) {
            cachedDepth = nativeDepth(INSTANCE_PTR);
        }

        return cachedDepth;
    }

    @Override
    protected void finalize() throws Throwable {
        delete();

        super.finalize();
    }

    private static native long nativeCreate(int[] seed, int index, int count, int security);

    private static native long nativeDepth(long ptr);

    private static native long nativeSize(long ptr);

    private static native void nativeDrop(long ptr);

    private static native int[] nativeSlice(long ptr);

    private static native long nativeBranch(long ptr, int index);
}
