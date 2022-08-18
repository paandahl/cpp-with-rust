#include "multibuf.h"

namespace org::blobstore {
    bool MultiBuf::has_more_chunks() const {
        return pos < chunks.size();
    }

    std::vector<uint8_t>& MultiBuf::next_chunk() {
        auto& next = chunks.at(pos);
        pos += 1;
        return next;
    }
}
