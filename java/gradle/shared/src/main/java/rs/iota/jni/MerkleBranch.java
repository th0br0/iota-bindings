package rs.iota.jni;

public class MerkleBranch {
    private long INSTANCE_PTR;

    private int index;
    private long cachedLength = -1;
    private int[] cachedSiblings = null;

    MerkleBranch(long ptr, int index) {
        this.INSTANCE_PTR = ptr;
        this.index = index;
    }

    public long length() {
        if (cachedLength == -1) {
            cachedLength = nativeLength(INSTANCE_PTR);
        }

        return cachedLength;
    }

    public int[] siblings() {
        if (cachedSiblings == null) {
            cachedSiblings = nativeSiblings(INSTANCE_PTR);
        }

        return cachedSiblings.clone();
    }

    public void delete() {
        if (INSTANCE_PTR != 0) {
            nativeDrop(INSTANCE_PTR);
            INSTANCE_PTR = 0;
        }
    }

    @Override
    protected void finalize() throws Throwable {
        delete();
        super.finalize();
    }

    private native static void nativeDrop(long ptr);

    private native static long nativeLength(long ptr);

    private native static int[] nativeSiblings(long ptr);
}
