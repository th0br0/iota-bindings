package rs.iota.jni;

public final class MAM {
    public static class DecodeResult {
        private final int[] message;
        private final int[] nextRoot;

        public DecodeResult(int[] message, int[] nextRoot) {
            this.message = message;
            this.nextRoot = nextRoot;
        }

        public int[] getMessage() {
            return message;
        }

        public int[] getNextRoot() {
            return nextRoot;
        }
    }

    ;

    public static int[] encode(
            int[] seed,
            int[] message,
            int[] sideKey,
            int[] root,
            int[] siblings,
            int[] nextRoot,
            int start, int index, int security) {
        if (seed == null || seed.length % 243 != 0) {
            throw new IllegalArgumentException("seed");
        }

        if (sideKey == null || sideKey.length != 243) {
            throw new IllegalArgumentException("sideKey");
        }

        if (root == null || root.length != 243) {
            throw new IllegalArgumentException("root");
        }

        if (nextRoot == null || nextRoot.length != 243) {
            throw new IllegalArgumentException("nextRoot");
        }

        if (message == null || siblings == null) {
            throw new IllegalArgumentException("message or siblings");
        }

        if (!(start >= 0 && index >= 0 && security > 0 && security <= 3)) {
            throw new IllegalArgumentException("start, index, security");
        }

        return nativeEncode(seed, message, sideKey, root, siblings, nextRoot, start, index, security);
    }

    public static native int[] nativeEncode(
            int[] seed,
            int[] message,
            int[] sideKey,
            int[] root,
            int[] siblings,
            int[] nextRoot,
            int start, int index, int security
    );

    public static DecodeResult decode(
            int[] encodedMessage,
            int[] sideKey,
            int[] root) throws Exception {
        if (encodedMessage == null) {
            throw new IllegalArgumentException("encodedMessage");
        }
        if (sideKey == null || sideKey.length != 243) {
            throw new IllegalArgumentException("sideKey");
        }
        if (root == null || root.length != 243) {
            throw new IllegalArgumentException("root");
        }

        return nativeDecode(encodedMessage, sideKey, root);
    }

    public static native DecodeResult nativeDecode(
            int[] encodedMessage,
            int[] sideKey,
            int[] root
    ) throws Exception;
}
